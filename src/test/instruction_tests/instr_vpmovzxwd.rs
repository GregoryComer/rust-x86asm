use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 200], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 25], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 219], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 2078044932, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 188, 119, 4, 115, 220, 123], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 214], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 681525786, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 180, 248, 26, 66, 159, 40], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 192], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 36, 242], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 51, 229], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 717119054, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 51, 132, 155, 78, 94, 190, 42], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 142, 51, 196], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 788112229, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 51, 44, 117, 101, 163, 249, 46], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 51, 238], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDI, 1976448703, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 51, 143, 191, 54, 206, 117], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 125, 169, 51, 252], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM22)), operand2: Some(IndirectDisplaced(RDX, 442942384, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 174, 51, 178, 176, 195, 102, 26], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 51, 202], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 51, 32], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 125, 207, 51, 244], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 51, 7], OperandSize::Qword)
}

