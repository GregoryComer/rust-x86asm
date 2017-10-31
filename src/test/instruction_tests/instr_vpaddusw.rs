use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 221, 244], OperandSize::Dword)
}

#[test]
fn vpaddusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 221, 8], OperandSize::Dword)
}

#[test]
fn vpaddusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 221, 242], OperandSize::Qword)
}

#[test]
fn vpaddusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 909545591, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 221, 4, 205, 119, 144, 54, 54], OperandSize::Qword)
}

#[test]
fn vpaddusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 221, 255], OperandSize::Dword)
}

#[test]
fn vpaddusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1565532626, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 221, 4, 69, 210, 33, 80, 93], OperandSize::Dword)
}

#[test]
fn vpaddusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 221, 210], OperandSize::Qword)
}

#[test]
fn vpaddusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 221, 60, 246], OperandSize::Qword)
}

#[test]
fn vpaddusw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 221, 249], OperandSize::Dword)
}

#[test]
fn vpaddusw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 1113957228, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 140, 221, 162, 108, 163, 101, 66], OperandSize::Dword)
}

#[test]
fn vpaddusw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 5, 131, 221, 236], OperandSize::Qword)
}

#[test]
fn vpaddusw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RAX, 1844193470, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 21, 142, 221, 152, 190, 40, 236, 109], OperandSize::Qword)
}

#[test]
fn vpaddusw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 174, 221, 195], OperandSize::Dword)
}

#[test]
fn vpaddusw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1402746945, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 221, 52, 197, 65, 56, 156, 83], OperandSize::Dword)
}

#[test]
fn vpaddusw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 125, 175, 221, 216], OperandSize::Qword)
}

#[test]
fn vpaddusw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 29, 175, 221, 30], OperandSize::Qword)
}

#[test]
fn vpaddusw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 205, 221, 207], OperandSize::Dword)
}

#[test]
fn vpaddusw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 221, 33], OperandSize::Dword)
}

#[test]
fn vpaddusw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 207, 221, 218], OperandSize::Qword)
}

#[test]
fn vpaddusw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 109, 197, 221, 6], OperandSize::Qword)
}

