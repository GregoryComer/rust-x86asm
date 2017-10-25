use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 232, 198], OperandSize::Dword)
}

#[test]
fn vpsubsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 232, 7], OperandSize::Dword)
}

#[test]
fn vpsubsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 232, 207], OperandSize::Qword)
}

#[test]
fn vpsubsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RBX, 980066705, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 232, 155, 145, 161, 106, 58], OperandSize::Qword)
}

#[test]
fn vpsubsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 232, 250], OperandSize::Dword)
}

#[test]
fn vpsubsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 1565606014, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 232, 135, 126, 64, 81, 93], OperandSize::Dword)
}

#[test]
fn vpsubsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 232, 218], OperandSize::Qword)
}

#[test]
fn vpsubsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDI, 736170025, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 232, 167, 41, 16, 225, 43], OperandSize::Qword)
}

#[test]
fn vpsubsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 232, 236], OperandSize::Dword)
}

#[test]
fn vpsubsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1637398325, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 232, 52, 93, 53, 183, 152, 97], OperandSize::Dword)
}

#[test]
fn vpsubsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 37, 143, 232, 215], OperandSize::Qword)
}

#[test]
fn vpsubsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 53, 133, 232, 8], OperandSize::Qword)
}

#[test]
fn vpsubsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 232, 201], OperandSize::Dword)
}

#[test]
fn vpsubsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 174, 232, 4, 240], OperandSize::Dword)
}

#[test]
fn vpsubsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 125, 172, 232, 197], OperandSize::Qword)
}

#[test]
fn vpsubsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM28)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 29, 166, 232, 57], OperandSize::Qword)
}

#[test]
fn vpsubsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 203, 232, 253], OperandSize::Dword)
}

#[test]
fn vpsubsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 201, 232, 57], OperandSize::Dword)
}

#[test]
fn vpsubsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 77, 197, 232, 221], OperandSize::Qword)
}

#[test]
fn vpsubsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1092919849, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 21, 202, 232, 28, 77, 41, 162, 36, 65], OperandSize::Qword)
}

