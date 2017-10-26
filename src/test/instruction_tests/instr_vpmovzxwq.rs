use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 230], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 964787693, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 172, 144, 237, 125, 129, 57], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 219], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 8], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 245], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EDX, 2091017440, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 178, 224, 100, 162, 124], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 222], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 52, 66], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 52, 250], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1817830997, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 52, 52, 189, 85, 230, 89, 108], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 125, 137, 52, 249], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM13)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 52, 42], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 52, 227], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EDI, 94246866, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 52, 151, 210, 23, 158, 5], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM21)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 125, 171, 52, 237], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 171, 52, 44, 152], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 52, 255], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 181874826, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 52, 156, 208, 138, 48, 215, 10], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 205, 52, 234], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectDisplaced(RSI, 1643174066, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 205, 52, 190, 178, 216, 240, 97], OperandSize::Qword)
}

