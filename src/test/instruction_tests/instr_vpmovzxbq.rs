use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 242], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1800063786, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 52, 133, 42, 203, 74, 107], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 205], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 60, 241], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 226], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 25], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 230], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 747512637, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 60, 189, 61, 35, 142, 44], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 50, 247], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 50, 20, 64], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 125, 142, 50, 219], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RBX, 21650721, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 50, 179, 33, 93, 74, 1], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 50, 238], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 50, 20, 198], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 125, 174, 50, 225], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 173, 50, 60, 177], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 50, 216], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EBX, 1380424564, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 50, 131, 116, 155, 71, 82], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 125, 201, 50, 227], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(RCX, 512344158, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 50, 185, 94, 192, 137, 30], OperandSize::Qword)
}

