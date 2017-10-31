use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 21, 201], OperandSize::Dword)
}

#[test]
fn vpmovusqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1203213548, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 21, 28, 93, 236, 148, 183, 71], OperandSize::Dword)
}

#[test]
fn vpmovusqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 126, 143, 21, 209], OperandSize::Qword)
}

#[test]
fn vpmovusqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledDisplaced(RCX, Four, 1415953283, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 21, 20, 141, 131, 187, 101, 84], OperandSize::Qword)
}

#[test]
fn vpmovusqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 172, 21, 231], OperandSize::Dword)
}

#[test]
fn vpmovusqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 21, 20, 82], OperandSize::Dword)
}

#[test]
fn vpmovusqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 126, 174, 21, 250], OperandSize::Qword)
}

#[test]
fn vpmovusqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 21, 38], OperandSize::Qword)
}

#[test]
fn vpmovusqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 21, 253], OperandSize::Dword)
}

#[test]
fn vpmovusqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1525827439, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 21, 132, 254, 111, 71, 242, 90], OperandSize::Dword)
}

#[test]
fn vpmovusqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 126, 204, 21, 199], OperandSize::Qword)
}

#[test]
fn vpmovusqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 21, 52, 241], OperandSize::Qword)
}

