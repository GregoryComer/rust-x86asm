use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 224], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 864688108, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 176, 236, 23, 138, 51], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 193], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 169425545, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 140, 113, 137, 58, 25, 10], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 202], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 27], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 249], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 52, 71], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 48, 244], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 48, 52, 81], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 125, 138, 48, 198], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RAX, 1418926624, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 48, 152, 32, 26, 147, 84], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 48, 226], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 48, 44, 195], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM21)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 125, 172, 48, 237], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM18)), operand2: Some(IndirectDisplaced(RBX, 2142442812, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 175, 48, 147, 60, 21, 179, 127], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 48, 245], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 48, 52, 201], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 125, 207, 48, 197], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM13)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 204, 48, 43], OperandSize::Qword)
}

