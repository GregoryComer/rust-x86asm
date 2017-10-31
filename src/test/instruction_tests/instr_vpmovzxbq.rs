use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 245], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 63935415, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 36, 157, 183, 147, 207, 3], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 206], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1721676999, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 44, 245, 199, 180, 158, 102], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 215], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 2037528816, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 164, 191, 240, 56, 114, 121], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 240], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RBX, 1816335742, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 179, 126, 21, 67, 108], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 50, 204], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 761744593, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 50, 52, 205, 209, 76, 103, 45], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 125, 141, 50, 201], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 228417486, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 50, 188, 70, 206, 95, 157, 13], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 50, 245], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 50, 52, 155], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 125, 175, 50, 232], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 175, 50, 36, 199], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 50, 202], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1535056211, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 50, 148, 217, 83, 25, 127, 91], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 206, 50, 215], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM27)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 205, 50, 31], OperandSize::Qword)
}

