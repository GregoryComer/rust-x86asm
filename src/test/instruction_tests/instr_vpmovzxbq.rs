use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 195], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 28, 72], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 237], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 170395583, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 4, 213, 191, 7, 40, 10], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 219], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(ESI, 62129486, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 134, 78, 5, 180, 3], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 227], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 28, 81], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 50, 251], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 50, 2], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 125, 143, 50, 212], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM9)), operand2: Some(IndirectDisplaced(RSI, 844473122, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 140, 50, 142, 34, 163, 85, 50], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 50, 246], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 50, 20, 251], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 125, 169, 50, 247], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 205721558, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 171, 50, 180, 122, 214, 15, 67, 12], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 50, 204], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1099036868, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 50, 60, 133, 196, 248, 129, 65], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 125, 204, 50, 254], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 201938903, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 50, 36, 221, 215, 87, 9, 12], OperandSize::Qword)
}

