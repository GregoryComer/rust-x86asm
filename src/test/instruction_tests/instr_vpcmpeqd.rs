use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 118, 252], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 675358051, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 118, 148, 176, 99, 37, 65, 40], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 118, 247], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 2020340914, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 118, 28, 157, 178, 244, 107, 120], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 118, 216], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1317753582, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 118, 12, 253, 238, 82, 139, 78], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 118, 212], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 449388958, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 118, 164, 112, 158, 33, 201, 26], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 15, 118, 229], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 599948756, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 14, 118, 145, 212, 125, 194, 35], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 29, 118, 16], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 93, 2, 118, 244], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 2021959597, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 5, 15, 118, 20, 85, 173, 167, 132, 120], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 943924842, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 13, 28, 118, 60, 253, 106, 38, 67, 56], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 43, 118, 236], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 1716177715, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 47, 118, 162, 51, 203, 74, 102], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 834322652, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 69, 61, 118, 145, 220, 192, 186, 49], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 42, 118, 201], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1271381519, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 61, 33, 118, 172, 191, 15, 190, 199, 75], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 102486629, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 45, 59, 118, 188, 182, 101, 210, 27, 6], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 78, 118, 204], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 76, 118, 25], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 725422989, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 117, 92, 118, 140, 137, 141, 19, 61, 43], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 21, 74, 118, 252], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectDisplaced(RDI, 1461460586, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 53, 75, 118, 167, 106, 30, 28, 87], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1707649943, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 101, 94, 118, 28, 181, 151, 171, 200, 101], OperandSize::Qword)
}

