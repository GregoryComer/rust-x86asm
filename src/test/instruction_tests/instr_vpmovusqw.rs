use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 139, 20, 247], OperandSize::Dword)
}

#[test]
fn vpmovusqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 20, 4, 194], OperandSize::Dword)
}

#[test]
fn vpmovusqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 126, 141, 20, 209], OperandSize::Qword)
}

#[test]
fn vpmovusqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 458792353, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 20, 188, 95, 161, 157, 88, 27], OperandSize::Qword)
}

#[test]
fn vpmovusqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 20, 217], OperandSize::Dword)
}

#[test]
fn vpmovusqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectDisplaced(ECX, 1419500994, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 20, 145, 194, 221, 155, 84], OperandSize::Dword)
}

#[test]
fn vpmovusqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 126, 171, 20, 207], OperandSize::Qword)
}

#[test]
fn vpmovusqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 20, 9], OperandSize::Qword)
}

#[test]
fn vpmovusqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 20, 206], OperandSize::Dword)
}

#[test]
fn vpmovusqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 20, 4, 154], OperandSize::Dword)
}

#[test]
fn vpmovusqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 126, 206, 20, 192], OperandSize::Qword)
}

#[test]
fn vpmovusqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectDisplaced(RDI, 774345197, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 20, 159, 237, 145, 39, 46], OperandSize::Qword)
}

