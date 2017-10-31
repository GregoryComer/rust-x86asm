use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 32, 192], OperandSize::Dword)
}

#[test]
fn vpmovswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 32, 2], OperandSize::Dword)
}

#[test]
fn vpmovswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 126, 141, 32, 198], OperandSize::Qword)
}

#[test]
fn vpmovswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 32, 44, 182], OperandSize::Qword)
}

#[test]
fn vpmovswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 32, 197], OperandSize::Dword)
}

#[test]
fn vpmovswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 32, 52, 158], OperandSize::Dword)
}

#[test]
fn vpmovswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM10)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 126, 172, 32, 210], OperandSize::Qword)
}

#[test]
fn vpmovswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1706866892, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 32, 44, 93, 204, 184, 188, 101], OperandSize::Qword)
}

#[test]
fn vpmovswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 32, 221], OperandSize::Dword)
}

#[test]
fn vpmovswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1418964844, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 32, 140, 217, 108, 175, 147, 84], OperandSize::Dword)
}

#[test]
fn vpmovswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(YMM21)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 126, 205, 32, 237], OperandSize::Qword)
}

#[test]
fn vpmovswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1697701323, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 32, 164, 114, 203, 221, 48, 101], OperandSize::Qword)
}

