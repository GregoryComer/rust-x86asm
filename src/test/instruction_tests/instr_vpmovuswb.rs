use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 16, 235], OperandSize::Dword)
}

#[test]
fn vpmovuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 16, 52, 195], OperandSize::Dword)
}

#[test]
fn vpmovuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 126, 141, 16, 232], OperandSize::Qword)
}

#[test]
fn vpmovuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectDisplaced(RBX, 662259483, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 16, 163, 27, 71, 121, 39], OperandSize::Qword)
}

#[test]
fn vpmovuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 172, 16, 238], OperandSize::Dword)
}

#[test]
fn vpmovuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 16, 20, 120], OperandSize::Dword)
}

#[test]
fn vpmovuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM18)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 126, 169, 16, 234], OperandSize::Qword)
}

#[test]
fn vpmovuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 729066193, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 16, 12, 245, 209, 170, 116, 43], OperandSize::Qword)
}

#[test]
fn vpmovuswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 204, 16, 245], OperandSize::Dword)
}

#[test]
fn vpmovuswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectDisplaced(ECX, 717102672, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 16, 129, 80, 30, 190, 42], OperandSize::Dword)
}

#[test]
fn vpmovuswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(YMM27)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 126, 204, 16, 243], OperandSize::Qword)
}

#[test]
fn vpmovuswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 16, 42], OperandSize::Qword)
}

