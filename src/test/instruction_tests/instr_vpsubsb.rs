use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 232, 238], OperandSize::Dword)
}

#[test]
fn vpsubsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EAX, 2056196581, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 232, 184, 229, 17, 143, 122], OperandSize::Dword)
}

#[test]
fn vpsubsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 232, 208], OperandSize::Qword)
}

#[test]
fn vpsubsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RAX, 1746661437, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 232, 128, 61, 240, 27, 104], OperandSize::Qword)
}

#[test]
fn vpsubsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 232, 229], OperandSize::Dword)
}

#[test]
fn vpsubsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1511483888, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 232, 36, 245, 240, 105, 23, 90], OperandSize::Dword)
}

#[test]
fn vpsubsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 232, 242], OperandSize::Qword)
}

#[test]
fn vpsubsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 232, 52, 246], OperandSize::Qword)
}

#[test]
fn vpsubsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 138, 232, 236], OperandSize::Dword)
}

#[test]
fn vpsubsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 625101437, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 137, 232, 131, 125, 74, 66, 37], OperandSize::Dword)
}

#[test]
fn vpsubsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 101, 133, 232, 220], OperandSize::Qword)
}

#[test]
fn vpsubsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RBX, 1630286250, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 125, 130, 232, 171, 170, 49, 44, 97], OperandSize::Qword)
}

#[test]
fn vpsubsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 172, 232, 195], OperandSize::Dword)
}

#[test]
fn vpsubsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 1221825444, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 232, 142, 164, 147, 211, 72], OperandSize::Dword)
}

#[test]
fn vpsubsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 109, 175, 232, 203], OperandSize::Qword)
}

#[test]
fn vpsubsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1085879264, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 53, 166, 232, 36, 213, 224, 51, 185, 64], OperandSize::Qword)
}

#[test]
fn vpsubsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 202, 232, 207], OperandSize::Dword)
}

#[test]
fn vpsubsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 126941167, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 204, 232, 20, 125, 239, 247, 144, 7], OperandSize::Dword)
}

#[test]
fn vpsubsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 53, 203, 232, 247], OperandSize::Qword)
}

#[test]
fn vpsubsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM13)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 21, 204, 232, 14], OperandSize::Qword)
}

