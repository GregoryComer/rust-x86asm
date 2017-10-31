use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 253, 222], OperandSize::Dword)
}

#[test]
fn vpaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 253, 52, 122], OperandSize::Dword)
}

#[test]
fn vpaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 253, 216], OperandSize::Qword)
}

#[test]
fn vpaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 2140003191, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 253, 4, 69, 119, 219, 141, 127], OperandSize::Qword)
}

#[test]
fn vpaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 253, 239], OperandSize::Dword)
}

#[test]
fn vpaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 253, 52, 186], OperandSize::Dword)
}

#[test]
fn vpaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 253, 246], OperandSize::Qword)
}

#[test]
fn vpaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RBX, 433723041, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 253, 187, 161, 22, 218, 25], OperandSize::Qword)
}

#[test]
fn vpaddw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 138, 253, 241], OperandSize::Dword)
}

#[test]
fn vpaddw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 139, 253, 39], OperandSize::Dword)
}

#[test]
fn vpaddw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 109, 134, 253, 208], OperandSize::Qword)
}

#[test]
fn vpaddw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 29, 133, 253, 19], OperandSize::Qword)
}

#[test]
fn vpaddw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 253, 226], OperandSize::Dword)
}

#[test]
fn vpaddw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1821883920, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 174, 253, 140, 207, 16, 190, 151, 108], OperandSize::Dword)
}

#[test]
fn vpaddw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 29, 170, 253, 247], OperandSize::Qword)
}

#[test]
fn vpaddw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1201762018, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 109, 166, 253, 164, 66, 226, 110, 161, 71], OperandSize::Qword)
}

