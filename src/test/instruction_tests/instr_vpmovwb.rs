use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovwb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 48, 246], OperandSize::Dword)
}

#[test]
fn vpmovwb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 48, 28, 250], OperandSize::Dword)
}

#[test]
fn vpmovwb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 126, 143, 48, 225], OperandSize::Qword)
}

#[test]
fn vpmovwb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 48, 39], OperandSize::Qword)
}

#[test]
fn vpmovwb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 48, 192], OperandSize::Dword)
}

#[test]
fn vpmovwb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectDisplaced(EBX, 794001203, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 48, 187, 51, 127, 83, 47], OperandSize::Dword)
}

#[test]
fn vpmovwb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM24)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 126, 173, 48, 248], OperandSize::Qword)
}

#[test]
fn vpmovwb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1397041653, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 48, 4, 221, 245, 41, 69, 83], OperandSize::Qword)
}

#[test]
fn vpmovwb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 48, 202], OperandSize::Dword)
}

#[test]
fn vpmovwb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 48, 62], OperandSize::Dword)
}

#[test]
fn vpmovwb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(YMM28)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 126, 206, 48, 236], OperandSize::Qword)
}

#[test]
fn vpmovwb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 48, 41], OperandSize::Qword)
}

