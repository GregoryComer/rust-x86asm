use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 205], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1207879467, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 156, 89, 43, 199, 254, 71], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 227], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 890517171, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 140, 82, 179, 54, 20, 53], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 237], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 31], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 240], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(RSI, 102197703, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 134, 199, 105, 23, 6], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 52, 228], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 750490069, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 52, 44, 69, 213, 145, 187, 44], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 125, 141, 52, 203], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM17)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 140, 52, 11], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 52, 249], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ESI, 1749118899, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 52, 158, 179, 111, 65, 104], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 52, 211], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1005879389, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 175, 52, 20, 69, 93, 128, 244, 59], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 52, 240], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 52, 20, 206], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 125, 207, 52, 248], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 720797438, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 201, 52, 12, 69, 254, 126, 246, 42], OperandSize::Qword)
}

