use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpestri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 222, 41], OperandSize::Dword)
}

fn pcmpestri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 2137315725, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 136, 141, 217, 100, 127, 72], OperandSize::Dword)
}

fn pcmpestri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 197, 47], OperandSize::Qword)
}

fn pcmpestri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 1783120372, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 176, 244, 65, 72, 106, 54], OperandSize::Qword)
}

