use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 232, 234], OperandSize::Dword)
}

#[test]
fn vpsubsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1966865192, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 232, 4, 93, 40, 251, 59, 117], OperandSize::Dword)
}

#[test]
fn vpsubsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 232, 212], OperandSize::Qword)
}

#[test]
fn vpsubsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1830328357, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 232, 188, 215, 37, 152, 24, 109], OperandSize::Qword)
}

#[test]
fn vpsubsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 232, 195], OperandSize::Dword)
}

#[test]
fn vpsubsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 608019688, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 232, 12, 181, 232, 164, 61, 36], OperandSize::Dword)
}

#[test]
fn vpsubsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 232, 215], OperandSize::Qword)
}

#[test]
fn vpsubsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 232, 6], OperandSize::Qword)
}

#[test]
fn vpsubsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 137, 232, 253], OperandSize::Dword)
}

#[test]
fn vpsubsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1933668572, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 232, 20, 221, 220, 112, 65, 115], OperandSize::Dword)
}

#[test]
fn vpsubsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 45, 142, 232, 227], OperandSize::Qword)
}

#[test]
fn vpsubsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 329830503, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 130, 232, 4, 221, 103, 208, 168, 19], OperandSize::Qword)
}

#[test]
fn vpsubsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 232, 208], OperandSize::Dword)
}

#[test]
fn vpsubsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 172, 232, 22], OperandSize::Dword)
}

#[test]
fn vpsubsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 45, 175, 232, 209], OperandSize::Qword)
}

#[test]
fn vpsubsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1874400869, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 125, 162, 232, 164, 219, 101, 22, 185, 111], OperandSize::Qword)
}

#[test]
fn vpsubsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 203, 232, 215], OperandSize::Dword)
}

#[test]
fn vpsubsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 2011165832, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 232, 36, 253, 136, 244, 223, 119], OperandSize::Dword)
}

#[test]
fn vpsubsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 85, 194, 232, 206], OperandSize::Qword)
}

#[test]
fn vpsubsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 872178863, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 93, 198, 232, 164, 138, 175, 100, 252, 51], OperandSize::Qword)
}

