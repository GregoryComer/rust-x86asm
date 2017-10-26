use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 221, 221], OperandSize::Dword)
}

#[test]
fn vpaddusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 221, 46], OperandSize::Dword)
}

#[test]
fn vpaddusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 221, 193], OperandSize::Qword)
}

#[test]
fn vpaddusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 450788890, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 221, 60, 69, 26, 126, 222, 26], OperandSize::Qword)
}

#[test]
fn vpaddusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 221, 210], OperandSize::Dword)
}

#[test]
fn vpaddusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 221, 63], OperandSize::Dword)
}

#[test]
fn vpaddusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 221, 200], OperandSize::Qword)
}

#[test]
fn vpaddusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 221, 51], OperandSize::Qword)
}

#[test]
fn vpaddusw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 137, 221, 213], OperandSize::Dword)
}

#[test]
fn vpaddusw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1669147746, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 137, 221, 36, 133, 98, 44, 125, 99], OperandSize::Dword)
}

#[test]
fn vpaddusw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 21, 135, 221, 238], OperandSize::Qword)
}

#[test]
fn vpaddusw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 13, 129, 221, 20, 79], OperandSize::Qword)
}

#[test]
fn vpaddusw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 170, 221, 235], OperandSize::Dword)
}

#[test]
fn vpaddusw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 221, 20, 137], OperandSize::Dword)
}

#[test]
fn vpaddusw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 45, 162, 221, 219], OperandSize::Qword)
}

#[test]
fn vpaddusw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 45, 170, 221, 51], OperandSize::Qword)
}

#[test]
fn vpaddusw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 206, 221, 223], OperandSize::Dword)
}

#[test]
fn vpaddusw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1071729649, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 221, 188, 198, 241, 75, 225, 63], OperandSize::Dword)
}

#[test]
fn vpaddusw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 85, 204, 221, 230], OperandSize::Qword)
}

#[test]
fn vpaddusw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1928627868, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 13, 195, 221, 156, 120, 156, 134, 244, 114], OperandSize::Qword)
}

