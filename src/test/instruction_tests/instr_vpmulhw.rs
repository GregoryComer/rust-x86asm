use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 229, 201], OperandSize::Dword)
}

#[test]
fn vpmulhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 1717239417, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 229, 172, 145, 121, 254, 90, 102], OperandSize::Dword)
}

#[test]
fn vpmulhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 229, 200], OperandSize::Qword)
}

#[test]
fn vpmulhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RAX, 1666730261, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 229, 176, 21, 73, 88, 99], OperandSize::Qword)
}

#[test]
fn vpmulhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 229, 254], OperandSize::Dword)
}

#[test]
fn vpmulhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 229, 12, 94], OperandSize::Dword)
}

#[test]
fn vpmulhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 229, 240], OperandSize::Qword)
}

#[test]
fn vpmulhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 518910846, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 229, 52, 189, 126, 243, 237, 30], OperandSize::Qword)
}

#[test]
fn vpmulhw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 138, 229, 226], OperandSize::Dword)
}

#[test]
fn vpmulhw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 229, 42], OperandSize::Dword)
}

#[test]
fn vpmulhw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 117, 129, 229, 222], OperandSize::Qword)
}

#[test]
fn vpmulhw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 53, 132, 229, 19], OperandSize::Qword)
}

#[test]
fn vpmulhw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 175, 229, 246], OperandSize::Dword)
}

#[test]
fn vpmulhw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1320594429, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 170, 229, 148, 112, 253, 171, 182, 78], OperandSize::Dword)
}

#[test]
fn vpmulhw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 45, 174, 229, 194], OperandSize::Qword)
}

#[test]
fn vpmulhw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RDX, 1591609621, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 13, 164, 229, 178, 21, 9, 222, 94], OperandSize::Qword)
}

#[test]
fn vpmulhw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 229, 212], OperandSize::Dword)
}

#[test]
fn vpmulhw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1126385457, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 207, 229, 12, 157, 49, 71, 35, 67], OperandSize::Dword)
}

#[test]
fn vpmulhw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 109, 205, 229, 252], OperandSize::Qword)
}

#[test]
fn vpmulhw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 21, 196, 229, 60, 246], OperandSize::Qword)
}

