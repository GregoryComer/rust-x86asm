use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 143, 21, 199], OperandSize::Dword)
}

#[test]
fn vpmovusqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 21, 36, 119], OperandSize::Dword)
}

#[test]
fn vpmovusqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 126, 143, 21, 236], OperandSize::Qword)
}

#[test]
fn vpmovusqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectDisplaced(RCX, 1976461812, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 21, 145, 244, 105, 206, 117], OperandSize::Qword)
}

#[test]
fn vpmovusqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 172, 21, 201], OperandSize::Dword)
}

#[test]
fn vpmovusqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1754377047, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 21, 28, 133, 87, 171, 145, 104], OperandSize::Dword)
}

#[test]
fn vpmovusqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 126, 169, 21, 254], OperandSize::Qword)
}

#[test]
fn vpmovusqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 2089566744, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 21, 36, 253, 24, 66, 140, 124], OperandSize::Qword)
}

#[test]
fn vpmovusqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 204, 21, 206], OperandSize::Dword)
}

#[test]
fn vpmovusqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1495041341, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 21, 28, 77, 61, 133, 28, 89], OperandSize::Dword)
}

#[test]
fn vpmovusqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 126, 206, 21, 197], OperandSize::Qword)
}

#[test]
fn vpmovusqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 21, 60, 222], OperandSize::Qword)
}

