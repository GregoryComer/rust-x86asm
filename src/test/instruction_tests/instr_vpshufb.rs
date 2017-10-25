use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 0, 227], OperandSize::Dword)
}

#[test]
fn vpshufb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 2033718049, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 0, 186, 33, 19, 56, 121], OperandSize::Dword)
}

#[test]
fn vpshufb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 0, 253], OperandSize::Qword)
}

#[test]
fn vpshufb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 0, 57], OperandSize::Qword)
}

#[test]
fn vpshufb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 0, 206], OperandSize::Dword)
}

#[test]
fn vpshufb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 460827033, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 0, 180, 154, 153, 169, 119, 27], OperandSize::Dword)
}

#[test]
fn vpshufb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 0, 230], OperandSize::Qword)
}

#[test]
fn vpshufb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1629183518, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 0, 12, 245, 30, 94, 27, 97], OperandSize::Qword)
}

#[test]
fn vpshufb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 0, 212], OperandSize::Dword)
}

#[test]
fn vpshufb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1302332863, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 0, 172, 135, 191, 5, 160, 77], OperandSize::Dword)
}

#[test]
fn vpshufb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 37, 137, 0, 238], OperandSize::Qword)
}

#[test]
fn vpshufb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1374582702, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 132, 0, 140, 206, 174, 119, 238, 81], OperandSize::Qword)
}

#[test]
fn vpshufb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 171, 0, 243], OperandSize::Dword)
}

#[test]
fn vpshufb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 849744671, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 0, 172, 146, 31, 19, 166, 50], OperandSize::Dword)
}

#[test]
fn vpshufb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 29, 161, 0, 253], OperandSize::Qword)
}

#[test]
fn vpshufb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 174, 0, 34], OperandSize::Qword)
}

#[test]
fn vpshufb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 205, 0, 223], OperandSize::Dword)
}

#[test]
fn vpshufb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 688192600, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 204, 0, 148, 135, 88, 252, 4, 41], OperandSize::Dword)
}

#[test]
fn vpshufb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 21, 194, 0, 210], OperandSize::Qword)
}

#[test]
fn vpshufb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RAX, 2135155485, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 69, 206, 0, 144, 29, 227, 67, 127], OperandSize::Qword)
}

