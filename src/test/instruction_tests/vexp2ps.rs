use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vexp2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 155, 200, 220], OperandSize::Dword)
}

fn vexp2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 200, 36, 128], OperandSize::Dword)
}

fn vexp2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 268826659, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 219, 200, 132, 218, 35, 248, 5, 16], OperandSize::Dword)
}

fn vexp2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 125, 155, 200, 245], OperandSize::Qword)
}

fn vexp2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectDisplaced(RSI, 1790521370, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 202, 200, 158, 26, 48, 185, 106], OperandSize::Qword)
}

fn vexp2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM20)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 218, 200, 34], OperandSize::Qword)
}

