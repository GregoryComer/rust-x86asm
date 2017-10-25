use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 221, 217], OperandSize::Dword)
}

#[test]
fn vpaddusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 36828498, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 221, 12, 117, 82, 245, 49, 2], OperandSize::Dword)
}

#[test]
fn vpaddusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 221, 236], OperandSize::Qword)
}

#[test]
fn vpaddusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1662516509, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 221, 148, 82, 29, 253, 23, 99], OperandSize::Qword)
}

#[test]
fn vpaddusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 221, 197], OperandSize::Dword)
}

#[test]
fn vpaddusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 2100743497, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 221, 4, 205, 73, 205, 54, 125], OperandSize::Dword)
}

#[test]
fn vpaddusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 221, 208], OperandSize::Qword)
}

#[test]
fn vpaddusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDX, 1276606691, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 221, 162, 227, 120, 23, 76], OperandSize::Qword)
}

#[test]
fn vpaddusw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 221, 246], OperandSize::Dword)
}

#[test]
fn vpaddusw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 138, 221, 62], OperandSize::Dword)
}

#[test]
fn vpaddusw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 5, 139, 221, 219], OperandSize::Qword)
}

#[test]
fn vpaddusw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1911309920, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 37, 132, 221, 44, 205, 96, 70, 236, 113], OperandSize::Qword)
}

#[test]
fn vpaddusw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 169, 221, 210], OperandSize::Dword)
}

#[test]
fn vpaddusw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 201497950, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 170, 221, 20, 141, 94, 157, 2, 12], OperandSize::Dword)
}

#[test]
fn vpaddusw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 5, 165, 221, 204], OperandSize::Qword)
}

#[test]
fn vpaddusw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 378864023, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 85, 164, 221, 20, 213, 151, 1, 149, 22], OperandSize::Qword)
}

#[test]
fn vpaddusw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 207, 221, 227], OperandSize::Dword)
}

#[test]
fn vpaddusw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1067428460, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 207, 221, 52, 141, 108, 170, 159, 63], OperandSize::Dword)
}

#[test]
fn vpaddusw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 221, 244], OperandSize::Qword)
}

#[test]
fn vpaddusw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(RDI, 1509584360, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 93, 205, 221, 151, 232, 109, 250, 89], OperandSize::Qword)
}

