use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 249, 195], OperandSize::Dword)
}

#[test]
fn vpsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 1076834941, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 249, 148, 137, 125, 50, 47, 64], OperandSize::Dword)
}

#[test]
fn vpsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 249, 230], OperandSize::Qword)
}

#[test]
fn vpsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 249, 2], OperandSize::Qword)
}

#[test]
fn vpsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 249, 254], OperandSize::Dword)
}

#[test]
fn vpsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 249, 25], OperandSize::Dword)
}

#[test]
fn vpsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 249, 203], OperandSize::Qword)
}

#[test]
fn vpsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 173929650, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 249, 180, 200, 178, 244, 93, 10], OperandSize::Qword)
}

#[test]
fn vpsubw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 249, 252], OperandSize::Dword)
}

#[test]
fn vpsubw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 143389086, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 143, 249, 158, 158, 241, 139, 8], OperandSize::Dword)
}

#[test]
fn vpsubw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 13, 132, 249, 233], OperandSize::Qword)
}

#[test]
fn vpsubw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RBX, 950005139, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 53, 141, 249, 147, 147, 237, 159, 56], OperandSize::Qword)
}

#[test]
fn vpsubw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 249, 229], OperandSize::Dword)
}

#[test]
fn vpsubw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1742584244, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 174, 249, 180, 73, 180, 185, 221, 103], OperandSize::Dword)
}

#[test]
fn vpsubw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 77, 161, 249, 214], OperandSize::Qword)
}

#[test]
fn vpsubw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 29, 164, 249, 36, 80], OperandSize::Qword)
}

#[test]
fn vpsubw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 249, 254], OperandSize::Dword)
}

#[test]
fn vpsubw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 201, 249, 28, 126], OperandSize::Dword)
}

#[test]
fn vpsubw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 125, 193, 249, 251], OperandSize::Qword)
}

#[test]
fn vpsubw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 29, 199, 249, 12, 241], OperandSize::Qword)
}

