use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 35, 229], OperandSize::Dword)
}

#[test]
fn vpmovsdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectDisplaced(EAX, 1338837653, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 35, 128, 149, 10, 205, 79], OperandSize::Dword)
}

#[test]
fn vpmovsdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 126, 139, 35, 239], OperandSize::Qword)
}

#[test]
fn vpmovsdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 35, 12, 119], OperandSize::Qword)
}

#[test]
fn vpmovsdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 172, 35, 240], OperandSize::Dword)
}

#[test]
fn vpmovsdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledDisplaced(ECX, Two, 649063118, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 35, 4, 77, 206, 234, 175, 38], OperandSize::Dword)
}

#[test]
fn vpmovsdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 126, 170, 35, 216], OperandSize::Qword)
}

#[test]
fn vpmovsdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 35, 48], OperandSize::Qword)
}

#[test]
fn vpmovsdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 35, 235], OperandSize::Dword)
}

#[test]
fn vpmovsdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1625052187, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 35, 4, 149, 27, 84, 220, 96], OperandSize::Dword)
}

#[test]
fn vpmovsdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(YMM24)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 126, 201, 35, 216], OperandSize::Qword)
}

#[test]
fn vpmovsdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectDisplaced(RDX, 899081779, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 35, 186, 51, 230, 150, 53], OperandSize::Qword)
}

