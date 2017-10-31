use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 229, 234], OperandSize::Dword)
}

#[test]
fn vpmulhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 1062271114, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 229, 190, 138, 248, 80, 63], OperandSize::Dword)
}

#[test]
fn vpmulhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 229, 225], OperandSize::Qword)
}

#[test]
fn vpmulhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 1109450863, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 229, 175, 111, 224, 32, 66], OperandSize::Qword)
}

#[test]
fn vpmulhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 229, 225], OperandSize::Dword)
}

#[test]
fn vpmulhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 2062134901, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 229, 156, 82, 117, 174, 233, 122], OperandSize::Dword)
}

#[test]
fn vpmulhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 229, 234], OperandSize::Qword)
}

#[test]
fn vpmulhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RSI, 1982138723, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 229, 182, 99, 9, 37, 118], OperandSize::Qword)
}

#[test]
fn vpmulhw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 229, 246], OperandSize::Dword)
}

#[test]
fn vpmulhw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1407864951, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 229, 132, 130, 119, 80, 234, 83], OperandSize::Dword)
}

#[test]
fn vpmulhw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 93, 142, 229, 196], OperandSize::Qword)
}

#[test]
fn vpmulhw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 132, 229, 28, 240], OperandSize::Qword)
}

#[test]
fn vpmulhw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 169, 229, 243], OperandSize::Dword)
}

#[test]
fn vpmulhw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EBX, 1738898398, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 169, 229, 163, 222, 123, 165, 103], OperandSize::Dword)
}

#[test]
fn vpmulhw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 93, 167, 229, 238], OperandSize::Qword)
}

#[test]
fn vpmulhw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 473637309, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 77, 164, 229, 156, 114, 189, 33, 59, 28], OperandSize::Qword)
}

#[test]
fn vpmulhw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 207, 229, 223], OperandSize::Dword)
}

#[test]
fn vpmulhw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 204, 229, 20, 130], OperandSize::Dword)
}

#[test]
fn vpmulhw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 61, 206, 229, 215], OperandSize::Qword)
}

#[test]
fn vpmulhw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RCX, 1424841162, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 37, 204, 229, 129, 202, 89, 237, 84], OperandSize::Qword)
}

