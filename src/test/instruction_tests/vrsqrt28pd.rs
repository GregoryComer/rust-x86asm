use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrsqrt28pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 156, 204, 244], OperandSize::Dword)
}

fn vrsqrt28pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1913422733, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 204, 60, 117, 141, 131, 12, 114], OperandSize::Dword)
}

fn vrsqrt28pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 889423158, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 204, 172, 130, 54, 133, 3, 53], OperandSize::Dword)
}

fn vrsqrt28pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 253, 154, 204, 204], OperandSize::Qword)
}

fn vrsqrt28pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 902006302, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 253, 206, 204, 188, 191, 30, 134, 195, 53], OperandSize::Qword)
}

fn vrsqrt28pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1071913946, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 219, 204, 156, 120, 218, 27, 228, 63], OperandSize::Qword)
}

