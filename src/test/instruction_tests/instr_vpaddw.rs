use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 253, 215], OperandSize::Dword)
}

#[test]
fn vpaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 2035596188, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 253, 156, 186, 156, 187, 84, 121], OperandSize::Dword)
}

#[test]
fn vpaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 253, 195], OperandSize::Qword)
}

#[test]
fn vpaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 253, 10], OperandSize::Qword)
}

#[test]
fn vpaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 253, 250], OperandSize::Dword)
}

#[test]
fn vpaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 253, 52, 199], OperandSize::Dword)
}

#[test]
fn vpaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 253, 213], OperandSize::Qword)
}

#[test]
fn vpaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 255328336, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 253, 156, 144, 80, 0, 56, 15], OperandSize::Qword)
}

#[test]
fn vpaddw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 141, 253, 193], OperandSize::Dword)
}

#[test]
fn vpaddw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 2065745903, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 137, 253, 164, 91, 239, 199, 32, 123], OperandSize::Dword)
}

#[test]
fn vpaddw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 69, 137, 253, 231], OperandSize::Qword)
}

#[test]
fn vpaddw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1742264619, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 143, 253, 28, 133, 43, 217, 216, 103], OperandSize::Qword)
}

#[test]
fn vpaddw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 253, 198], OperandSize::Dword)
}

#[test]
fn vpaddw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 253, 23], OperandSize::Dword)
}

#[test]
fn vpaddw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 85, 161, 253, 224], OperandSize::Qword)
}

#[test]
fn vpaddw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 53, 174, 253, 4, 191], OperandSize::Qword)
}

