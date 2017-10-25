use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 249, 238], OperandSize::Dword)
}

#[test]
fn vpsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 249, 22], OperandSize::Dword)
}

#[test]
fn vpsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 249, 252], OperandSize::Qword)
}

#[test]
fn vpsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 249, 12, 240], OperandSize::Qword)
}

#[test]
fn vpsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 249, 214], OperandSize::Dword)
}

#[test]
fn vpsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1422984831, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 249, 44, 181, 127, 6, 209, 84], OperandSize::Dword)
}

#[test]
fn vpsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 249, 255], OperandSize::Qword)
}

#[test]
fn vpsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 249, 52, 255], OperandSize::Qword)
}

#[test]
fn vpsubw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 249, 231], OperandSize::Dword)
}

#[test]
fn vpsubw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 613040642, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 137, 249, 52, 205, 2, 66, 138, 36], OperandSize::Dword)
}

#[test]
fn vpsubw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 109, 134, 249, 243], OperandSize::Qword)
}

#[test]
fn vpsubw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 269187335, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 85, 142, 249, 52, 93, 7, 121, 11, 16], OperandSize::Qword)
}

#[test]
fn vpsubw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 171, 249, 201], OperandSize::Dword)
}

#[test]
fn vpsubw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 946334823, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 173, 249, 172, 186, 103, 236, 103, 56], OperandSize::Dword)
}

#[test]
fn vpsubw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 125, 162, 249, 231], OperandSize::Qword)
}

#[test]
fn vpsubw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectDisplaced(RBX, 1728159590, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 69, 167, 249, 171, 102, 159, 1, 103], OperandSize::Qword)
}

#[test]
fn vpsubw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 249, 235], OperandSize::Dword)
}

#[test]
fn vpsubw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 698748149, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 206, 249, 172, 122, 245, 12, 166, 41], OperandSize::Dword)
}

#[test]
fn vpsubw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 53, 194, 249, 247], OperandSize::Qword)
}

#[test]
fn vpsubw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectDisplaced(RAX, 80355783, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 21, 193, 249, 184, 199, 33, 202, 4], OperandSize::Qword)
}

