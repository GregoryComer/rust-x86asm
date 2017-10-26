use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 41, 250], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 41, 28, 126], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 41, 235], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 937405084, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 41, 12, 245, 156, 170, 223, 55], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 41, 236], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 41, 36, 177], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 41, 213], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 41, 16], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 10, 41, 254], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 1735718197, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 14, 41, 185, 53, 245, 116, 103], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 510151884, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 31, 41, 12, 181, 204, 76, 104, 30], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 157, 6, 41, 249], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1058895290, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 10, 41, 140, 112, 186, 117, 29, 63], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1446888407, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 19, 41, 20, 149, 215, 195, 61, 86], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 43, 41, 201], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 42, 41, 12, 208], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 57, 41, 31], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 237, 44, 41, 202], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 35, 41, 44, 90], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 52, 41, 60, 81], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 76, 41, 231], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 2074734795, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 74, 41, 156, 120, 203, 240, 169, 123], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ECX, 349750131, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 91, 41, 185, 115, 195, 216, 20], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 173, 67, 41, 213], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectDisplaced(RAX, 675390030, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 181, 65, 41, 144, 78, 162, 65, 40], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1797539752, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 86, 41, 20, 125, 168, 71, 36, 107], OperandSize::Qword)
}

