use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 41, 248], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 41, 18], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 41, 249], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 41, 8], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 41, 232], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 608223602, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 41, 146, 114, 193, 64, 36], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 41, 208], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 741688930, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 41, 132, 82, 98, 70, 53, 44], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 11, 41, 201], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 1049883627, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 14, 41, 152, 235, 243, 147, 62], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 877682239, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 25, 41, 182, 63, 94, 80, 52], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 133, 10, 41, 217], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 89693781, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 189, 2, 41, 148, 199, 85, 158, 88, 5], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RBX, 519775701, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 133, 17, 41, 187, 213, 37, 251, 30], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 41, 41, 242], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 46, 41, 58], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 60, 41, 30], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 221, 38, 41, 239], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1523038232, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 45, 41, 20, 141, 24, 184, 199, 90], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 61, 41, 23], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 73, 41, 212], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1261430872, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 73, 41, 36, 221, 88, 232, 47, 75], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 91, 41, 54], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 229, 76, 41, 236], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 569116371, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 76, 41, 60, 157, 211, 6, 236, 33], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1215940676, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 91, 41, 36, 157, 68, 200, 121, 72], OperandSize::Qword)
}

