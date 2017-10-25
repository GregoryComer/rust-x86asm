use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn comisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 233], OperandSize::Dword)
}

fn comisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1266860078, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 4, 181, 46, 192, 130, 75], OperandSize::Dword)
}

fn comisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 209], OperandSize::Qword)
}

fn comisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 40], OperandSize::Qword)
}

