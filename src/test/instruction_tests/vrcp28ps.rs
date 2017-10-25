use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrcp28ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 156, 202, 211], OperandSize::Dword)
}

fn vrcp28ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 202, 4, 95], OperandSize::Dword)
}

fn vrcp28ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 202, 4, 210], OperandSize::Dword)
}

fn vrcp28ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 125, 155, 202, 215], OperandSize::Qword)
}

fn vrcp28ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 386447576, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 204, 202, 156, 159, 216, 184, 8, 23], OperandSize::Qword)
}

fn vrcp28ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PS, operand1: Some(Direct(ZMM26)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 218, 202, 17], OperandSize::Qword)
}

