use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpexpandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 137, 255], OperandSize::Dword)
}

#[test]
fn vpexpandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ECX, 1002720127, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 137, 137, 127, 75, 196, 59], OperandSize::Dword)
}

#[test]
fn vpexpandq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 142, 137, 222], OperandSize::Qword)
}

#[test]
fn vpexpandq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 506621228, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 137, 44, 85, 44, 109, 50, 30], OperandSize::Qword)
}

#[test]
fn vpexpandq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 137, 222], OperandSize::Dword)
}

#[test]
fn vpexpandq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 137, 30], OperandSize::Dword)
}

#[test]
fn vpexpandq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 253, 172, 137, 221], OperandSize::Qword)
}

#[test]
fn vpexpandq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM28)), operand2: Some(IndirectDisplaced(RCX, 616871947, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 253, 169, 137, 161, 11, 184, 196, 36], OperandSize::Qword)
}

#[test]
fn vpexpandq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 137, 254], OperandSize::Dword)
}

#[test]
fn vpexpandq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(ECX, 1379638909, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 137, 137, 125, 158, 59, 82], OperandSize::Dword)
}

#[test]
fn vpexpandq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 253, 205, 137, 239], OperandSize::Qword)
}

#[test]
fn vpexpandq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectDisplaced(RAX, 65978405, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 253, 204, 137, 136, 37, 192, 238, 3], OperandSize::Qword)
}

