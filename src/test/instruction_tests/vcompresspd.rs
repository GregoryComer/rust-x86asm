use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcompresspd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 138, 201], OperandSize::Dword)
}

fn vcompresspd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1600126759, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 8, 138, 132, 64, 39, 255, 95, 95], OperandSize::Dword)
}

fn vcompresspd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 253, 143, 138, 213], OperandSize::Qword)
}

fn vcompresspd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 8, 138, 12, 138], OperandSize::Qword)
}

fn vcompresspd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 138, 211], OperandSize::Dword)
}

fn vcompresspd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1148576162, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 40, 138, 4, 125, 162, 225, 117, 68], OperandSize::Dword)
}

fn vcompresspd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 253, 171, 138, 227], OperandSize::Qword)
}

fn vcompresspd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 663729546, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 253, 40, 138, 148, 186, 138, 181, 143, 39], OperandSize::Qword)
}

fn vcompresspd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 138, 235], OperandSize::Dword)
}

fn vcompresspd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1780854872, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 138, 60, 189, 88, 176, 37, 106], OperandSize::Dword)
}

fn vcompresspd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 253, 207, 138, 216], OperandSize::Qword)
}

fn vcompresspd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 253, 72, 138, 25], OperandSize::Qword)
}

