use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 253, 236], OperandSize::Dword)
}

#[test]
fn vpaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 253, 44, 201], OperandSize::Dword)
}

#[test]
fn vpaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 253, 245], OperandSize::Qword)
}

#[test]
fn vpaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 253, 1], OperandSize::Qword)
}

#[test]
fn vpaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 253, 202], OperandSize::Dword)
}

#[test]
fn vpaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 253, 24], OperandSize::Dword)
}

#[test]
fn vpaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 253, 225], OperandSize::Qword)
}

#[test]
fn vpaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 253, 12, 182], OperandSize::Qword)
}

#[test]
fn vpaddw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 141, 253, 253], OperandSize::Dword)
}

#[test]
fn vpaddw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1850724015, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 137, 253, 180, 178, 175, 206, 79, 110], OperandSize::Dword)
}

#[test]
fn vpaddw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 109, 142, 253, 252], OperandSize::Qword)
}

#[test]
fn vpaddw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 585747531, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 69, 131, 253, 140, 194, 75, 204, 233, 34], OperandSize::Qword)
}

#[test]
fn vpaddw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 172, 253, 203], OperandSize::Dword)
}

#[test]
fn vpaddw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1491709583, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 171, 253, 180, 87, 143, 174, 233, 88], OperandSize::Dword)
}

#[test]
fn vpaddw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 69, 174, 253, 208], OperandSize::Qword)
}

#[test]
fn vpaddw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 101, 163, 253, 12, 215], OperandSize::Qword)
}

