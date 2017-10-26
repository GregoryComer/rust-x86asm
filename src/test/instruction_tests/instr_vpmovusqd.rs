use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 21, 233], OperandSize::Dword)
}

#[test]
fn vpmovusqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 21, 28, 118], OperandSize::Dword)
}

#[test]
fn vpmovusqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 126, 139, 21, 242], OperandSize::Qword)
}

#[test]
fn vpmovusqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 21, 4, 202], OperandSize::Qword)
}

#[test]
fn vpmovusqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 21, 192], OperandSize::Dword)
}

#[test]
fn vpmovusqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1583323097, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 21, 140, 139, 217, 151, 95, 94], OperandSize::Dword)
}

#[test]
fn vpmovusqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 126, 172, 21, 242], OperandSize::Qword)
}

#[test]
fn vpmovusqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 30148404, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 21, 188, 194, 52, 7, 204, 1], OperandSize::Qword)
}

#[test]
fn vpmovusqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 21, 232], OperandSize::Dword)
}

#[test]
fn vpmovusqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 21, 2], OperandSize::Dword)
}

#[test]
fn vpmovusqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 126, 201, 21, 204], OperandSize::Qword)
}

#[test]
fn vpmovusqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 21, 54], OperandSize::Qword)
}

