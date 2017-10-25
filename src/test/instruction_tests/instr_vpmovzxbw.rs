use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 255], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1748329946, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 180, 112, 218, 101, 53, 104], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 224], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1705079266, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 172, 210, 226, 113, 161, 101], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 199], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1195164685, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 20, 205, 13, 196, 60, 71], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 246], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1103519193, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 28, 133, 217, 93, 198, 65], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 48, 252], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDX, 254666287, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 48, 146, 47, 230, 45, 15], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 125, 141, 48, 234], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 858944056, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 143, 48, 132, 129, 56, 114, 50, 51], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 48, 214], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 48, 28, 113], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 125, 170, 48, 194], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 173, 48, 36, 201], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 48, 222], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1165715993, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 48, 28, 77, 25, 106, 123, 69], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 125, 202, 48, 231], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectDisplaced(RBX, 1579555971, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 207, 48, 155, 131, 28, 38, 94], OperandSize::Qword)
}

