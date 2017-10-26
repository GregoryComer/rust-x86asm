use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 51, 241], OperandSize::Dword)
}

#[test]
fn vpmovdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 2137149301, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 51, 12, 205, 117, 79, 98, 127], OperandSize::Dword)
}

#[test]
fn vpmovdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 126, 143, 51, 208], OperandSize::Qword)
}

#[test]
fn vpmovdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 51, 58], OperandSize::Qword)
}

#[test]
fn vpmovdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 51, 222], OperandSize::Dword)
}

#[test]
fn vpmovdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectDisplaced(ESI, 1252825547, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 51, 190, 203, 153, 172, 74], OperandSize::Dword)
}

#[test]
fn vpmovdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 126, 171, 51, 214], OperandSize::Qword)
}

#[test]
fn vpmovdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectDisplaced(RDI, 383805186, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 51, 159, 2, 103, 224, 22], OperandSize::Qword)
}

#[test]
fn vpmovdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 202, 51, 228], OperandSize::Dword)
}

#[test]
fn vpmovdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 51, 60, 195], OperandSize::Dword)
}

#[test]
fn vpmovdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 126, 204, 51, 246], OperandSize::Qword)
}

#[test]
fn vpmovdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1758141262, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 51, 28, 205, 78, 27, 203, 104], OperandSize::Qword)
}

