use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 234, 233], OperandSize::Dword)
}

#[test]
fn vpminsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 1971539817, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 234, 166, 105, 79, 131, 117], OperandSize::Dword)
}

#[test]
fn vpminsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 234, 233], OperandSize::Qword)
}

#[test]
fn vpminsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 234, 20, 136], OperandSize::Qword)
}

#[test]
fn vpminsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 234, 197], OperandSize::Dword)
}

#[test]
fn vpminsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 234, 51], OperandSize::Dword)
}

#[test]
fn vpminsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 234, 195], OperandSize::Qword)
}

#[test]
fn vpminsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RCX, 884014817, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 234, 177, 225, 254, 176, 52], OperandSize::Qword)
}

#[test]
fn vpminsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 234, 222], OperandSize::Dword)
}

#[test]
fn vpminsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 234, 41], OperandSize::Dword)
}

#[test]
fn vpminsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 85, 137, 234, 219], OperandSize::Qword)
}

#[test]
fn vpminsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 2001444248, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 109, 129, 234, 132, 135, 152, 157, 75, 119], OperandSize::Qword)
}

#[test]
fn vpminsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 234, 198], OperandSize::Dword)
}

#[test]
fn vpminsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 234, 36, 88], OperandSize::Dword)
}

#[test]
fn vpminsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 13, 175, 234, 250], OperandSize::Qword)
}

#[test]
fn vpminsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 117, 171, 234, 12, 201], OperandSize::Qword)
}

#[test]
fn vpminsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 201, 234, 201], OperandSize::Dword)
}

#[test]
fn vpminsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 205, 234, 44, 88], OperandSize::Dword)
}

#[test]
fn vpminsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 29, 201, 234, 239], OperandSize::Qword)
}

#[test]
fn vpminsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM19)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 101, 198, 234, 50], OperandSize::Qword)
}

