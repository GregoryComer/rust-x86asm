use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vucomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 236], OperandSize::Dword)
}

fn vucomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 44, 246], OperandSize::Dword)
}

fn vucomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 214], OperandSize::Qword)
}

fn vucomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 24], OperandSize::Qword)
}

fn vucomisd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 253, 24, 46, 213], OperandSize::Dword)
}

fn vucomisd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 255085175, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 4, 125, 119, 74, 52, 15], OperandSize::Dword)
}

fn vucomisd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 209, 253, 24, 46, 243], OperandSize::Qword)
}

fn vucomisd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 69154331, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 8, 46, 140, 158, 27, 54, 31, 4], OperandSize::Qword)
}

