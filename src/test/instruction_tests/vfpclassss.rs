use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfpclassss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 12, 103, 236, 5], OperandSize::Dword)
}

fn vfpclassss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 862130471, Some(OperandSize::Dword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 15, 103, 140, 146, 39, 17, 99, 51, 72], OperandSize::Dword)
}

fn vfpclassss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM30)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 125, 11, 103, 206, 113], OperandSize::Qword)
}

fn vfpclassss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSS, operand1: Some(Direct(K5)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 14, 103, 41, 72], OperandSize::Qword)
}

