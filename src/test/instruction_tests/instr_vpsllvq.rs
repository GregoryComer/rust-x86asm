use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 71, 250], OperandSize::Dword)
}

#[test]
fn vpsllvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 1779458263, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 71, 137, 215, 96, 16, 106], OperandSize::Dword)
}

#[test]
fn vpsllvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 71, 251], OperandSize::Qword)
}

#[test]
fn vpsllvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 71, 59], OperandSize::Qword)
}

#[test]
fn vpsllvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 71, 238], OperandSize::Dword)
}

#[test]
fn vpsllvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 71, 18], OperandSize::Dword)
}

#[test]
fn vpsllvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 71, 239], OperandSize::Qword)
}

#[test]
fn vpsllvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 71, 22], OperandSize::Qword)
}

#[test]
fn vpsllvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 137, 71, 248], OperandSize::Dword)
}

#[test]
fn vpsllvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 21732853, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 140, 71, 151, 245, 157, 75, 1], OperandSize::Dword)
}

#[test]
fn vpsllvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 154, 71, 57], OperandSize::Dword)
}

#[test]
fn vpsllvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 157, 130, 71, 221], OperandSize::Qword)
}

#[test]
fn vpsllvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 2128728409, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 189, 138, 71, 188, 251, 89, 209, 225, 126], OperandSize::Qword)
}

#[test]
fn vpsllvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 141, 146, 71, 10], OperandSize::Qword)
}

#[test]
fn vpsllvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 175, 71, 207], OperandSize::Dword)
}

#[test]
fn vpsllvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 174, 71, 9], OperandSize::Dword)
}

#[test]
fn vpsllvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1271931946, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 190, 71, 185, 42, 36, 208, 75], OperandSize::Dword)
}

#[test]
fn vpsllvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 157, 167, 71, 221], OperandSize::Qword)
}

#[test]
fn vpsllvq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM8)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 189, 174, 71, 22], OperandSize::Qword)
}

#[test]
fn vpsllvq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1598278458, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 141, 182, 71, 44, 253, 58, 203, 67, 95], OperandSize::Qword)
}

#[test]
fn vpsllvq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 202, 71, 193], OperandSize::Dword)
}

#[test]
fn vpsllvq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 206, 71, 4, 152], OperandSize::Dword)
}

#[test]
fn vpsllvq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 223559384, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 222, 71, 188, 216, 216, 62, 83, 13], OperandSize::Dword)
}

#[test]
fn vpsllvq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 221, 202, 71, 216], OperandSize::Qword)
}

#[test]
fn vpsllvq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 194, 71, 4, 223], OperandSize::Qword)
}

#[test]
fn vpsllvq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1479649975, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 212, 71, 4, 253, 183, 170, 49, 88], OperandSize::Qword)
}

