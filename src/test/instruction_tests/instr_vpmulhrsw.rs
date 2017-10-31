use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhrsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 11, 208], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 674906474, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 11, 150, 106, 65, 58, 40], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 11, 197], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 11, 51], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 11, 214], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 314905607, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 11, 4, 85, 7, 20, 197, 18], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 11, 204], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RCX, 79211118, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 11, 129, 110, 170, 184, 4], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 143, 11, 229], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 7923456, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 11, 132, 95, 0, 231, 120, 0], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 109, 129, 11, 194], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1789541045, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 135, 11, 148, 151, 181, 58, 170, 106], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 11, 192], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EBX, 2103938039, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 11, 139, 247, 139, 103, 125], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 77, 162, 11, 222], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RSI, 392269466, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 109, 170, 11, 182, 154, 142, 97, 23], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 201, 11, 254], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 207, 11, 9], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 93, 202, 11, 215], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 37, 194, 11, 28, 209], OperandSize::Qword)
}

