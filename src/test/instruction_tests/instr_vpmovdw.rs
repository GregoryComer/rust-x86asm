use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 51, 227], OperandSize::Dword)
}

#[test]
fn vpmovdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectDisplaced(EDI, 2123967429, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 51, 191, 197, 43, 153, 126], OperandSize::Dword)
}

#[test]
fn vpmovdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 51, 211], OperandSize::Qword)
}

#[test]
fn vpmovdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectDisplaced(RDX, 429648538, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 51, 170, 154, 234, 155, 25], OperandSize::Qword)
}

#[test]
fn vpmovdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 51, 238], OperandSize::Dword)
}

#[test]
fn vpmovdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 2142143432, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 51, 148, 202, 200, 131, 174, 127], OperandSize::Dword)
}

#[test]
fn vpmovdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 126, 174, 51, 251], OperandSize::Qword)
}

#[test]
fn vpmovdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 51, 11], OperandSize::Qword)
}

#[test]
fn vpmovdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 51, 217], OperandSize::Dword)
}

#[test]
fn vpmovdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectDisplaced(EBX, 1493688364, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 51, 179, 44, 224, 7, 89], OperandSize::Dword)
}

#[test]
fn vpmovdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 126, 204, 51, 226], OperandSize::Qword)
}

#[test]
fn vpmovdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectDisplaced(RCX, 667462987, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 51, 129, 75, 173, 200, 39], OperandSize::Qword)
}

