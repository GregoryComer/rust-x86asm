use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpexpandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 137, 205], OperandSize::Dword)
}

#[test]
fn vpexpandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDX, 513503677, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 137, 186, 189, 113, 155, 30], OperandSize::Dword)
}

#[test]
fn vpexpandd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 137, 137, 207], OperandSize::Qword)
}

#[test]
fn vpexpandd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM17)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 143, 137, 9], OperandSize::Qword)
}

#[test]
fn vpexpandd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 137, 221], OperandSize::Dword)
}

#[test]
fn vpexpandd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 770282695, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 137, 52, 181, 199, 148, 233, 45], OperandSize::Dword)
}

#[test]
fn vpexpandd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 125, 175, 137, 223], OperandSize::Qword)
}

#[test]
fn vpexpandd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1353133071, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 137, 28, 205, 15, 44, 167, 80], OperandSize::Qword)
}

#[test]
fn vpexpandd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 137, 240], OperandSize::Dword)
}

#[test]
fn vpexpandd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 137, 60, 255], OperandSize::Dword)
}

#[test]
fn vpexpandd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 125, 204, 137, 252], OperandSize::Qword)
}

#[test]
fn vpexpandd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM25)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 201, 137, 9], OperandSize::Qword)
}

