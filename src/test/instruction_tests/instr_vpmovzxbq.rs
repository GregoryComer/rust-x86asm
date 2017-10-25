use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 247], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 826461665, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 145, 225, 205, 66, 49], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 230], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 50], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 219], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1851067885, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 172, 217, 237, 13, 85, 110], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 242], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 51], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 50, 233], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 1648666786, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 50, 154, 162, 168, 68, 98], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 125, 141, 50, 200], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1239712416, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 50, 52, 149, 160, 130, 228, 73], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 50, 227], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 318423659, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 50, 36, 245, 107, 194, 250, 18], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 125, 175, 50, 212], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 50, 12, 159], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 50, 243], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(EDI, 1710463299, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 50, 159, 67, 153, 243, 101], OperandSize::Dword)
}

#[test]
fn vpmovzxbq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 125, 206, 50, 225], OperandSize::Qword)
}

#[test]
fn vpmovzxbq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM19)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 202, 50, 27], OperandSize::Qword)
}

