use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 233, 220], OperandSize::Dword)
}

#[test]
fn vpsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 359456889, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 233, 44, 69, 121, 224, 108, 21], OperandSize::Dword)
}

#[test]
fn vpsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 233, 215], OperandSize::Qword)
}

#[test]
fn vpsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 154922505, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 233, 140, 129, 9, 238, 59, 9], OperandSize::Qword)
}

#[test]
fn vpsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 233, 193], OperandSize::Dword)
}

#[test]
fn vpsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 233, 35], OperandSize::Dword)
}

#[test]
fn vpsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 233, 194], OperandSize::Qword)
}

#[test]
fn vpsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 233, 17], OperandSize::Qword)
}

#[test]
fn vpsubsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 141, 233, 237], OperandSize::Dword)
}

#[test]
fn vpsubsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 676094806, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 143, 233, 172, 119, 86, 99, 76, 40], OperandSize::Dword)
}

#[test]
fn vpsubsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 109, 134, 233, 193], OperandSize::Qword)
}

#[test]
fn vpsubsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 45, 131, 233, 20, 182], OperandSize::Qword)
}

#[test]
fn vpsubsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 169, 233, 198], OperandSize::Dword)
}

#[test]
fn vpsubsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 2031122819, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 173, 233, 140, 187, 131, 121, 16, 121], OperandSize::Dword)
}

#[test]
fn vpsubsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 21, 174, 233, 244], OperandSize::Qword)
}

#[test]
fn vpsubsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RAX, 420132696, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 53, 164, 233, 128, 88, 183, 10, 25], OperandSize::Qword)
}

