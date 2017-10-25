use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vexp2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 155, 200, 214], OperandSize::Dword)
}

fn vexp2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 588795356, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 200, 12, 197, 220, 77, 24, 35], OperandSize::Dword)
}

fn vexp2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1283700834, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 200, 132, 72, 98, 184, 131, 76], OperandSize::Dword)
}

fn vexp2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 253, 155, 200, 227], OperandSize::Qword)
}

fn vexp2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM17)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 253, 207, 200, 14], OperandSize::Qword)
}

fn vexp2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1990891098, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 200, 164, 150, 90, 150, 170, 118], OperandSize::Qword)
}

