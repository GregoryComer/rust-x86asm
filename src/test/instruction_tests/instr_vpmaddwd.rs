use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaddwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 245, 234], OperandSize::Dword)
}

#[test]
fn vpmaddwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 745497996, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 245, 140, 112, 140, 101, 111, 44], OperandSize::Dword)
}

#[test]
fn vpmaddwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 245, 230], OperandSize::Qword)
}

#[test]
fn vpmaddwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RBX, 649979991, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 245, 131, 87, 232, 189, 38], OperandSize::Qword)
}

#[test]
fn vpmaddwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 245, 228], OperandSize::Dword)
}

#[test]
fn vpmaddwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 245, 41], OperandSize::Dword)
}

#[test]
fn vpmaddwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 245, 225], OperandSize::Qword)
}

#[test]
fn vpmaddwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 2093069807, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 245, 172, 190, 239, 181, 193, 124], OperandSize::Qword)
}

#[test]
fn vpmaddwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 245, 229], OperandSize::Dword)
}

#[test]
fn vpmaddwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1252085645, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 141, 245, 172, 75, 141, 79, 161, 74], OperandSize::Dword)
}

#[test]
fn vpmaddwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 61, 131, 245, 247], OperandSize::Qword)
}

#[test]
fn vpmaddwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1293501026, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 13, 132, 245, 4, 149, 98, 66, 25, 77], OperandSize::Qword)
}

#[test]
fn vpmaddwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 245, 252], OperandSize::Dword)
}

#[test]
fn vpmaddwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EBX, 746631636, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 245, 179, 212, 177, 128, 44], OperandSize::Dword)
}

#[test]
fn vpmaddwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 174, 245, 200], OperandSize::Qword)
}

#[test]
fn vpmaddwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM15)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 5, 174, 245, 27], OperandSize::Qword)
}

#[test]
fn vpmaddwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 207, 245, 225], OperandSize::Dword)
}

#[test]
fn vpmaddwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 207, 245, 52, 177], OperandSize::Dword)
}

#[test]
fn vpmaddwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 21, 202, 245, 209], OperandSize::Qword)
}

#[test]
fn vpmaddwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 194, 245, 47], OperandSize::Qword)
}

