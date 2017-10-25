use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrcp28pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 154, 202, 204], OperandSize::Dword)
}

fn vrcp28pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EDI, 2003609792, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 202, 191, 192, 168, 108, 119], OperandSize::Dword)
}

fn vrcp28pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1807149447, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 202, 12, 141, 135, 233, 182, 107], OperandSize::Dword)
}

fn vrcp28pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 253, 154, 202, 225], OperandSize::Qword)
}

fn vrcp28pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 202, 4, 176], OperandSize::Qword)
}

fn vrcp28pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28PD, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 202, 3], OperandSize::Qword)
}

