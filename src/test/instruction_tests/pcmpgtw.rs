use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpgtw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 226], OperandSize::Dword)
}

fn pcmpgtw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 41], OperandSize::Dword)
}

fn pcmpgtw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 237], OperandSize::Qword)
}

fn pcmpgtw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 797684365, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 20, 141, 141, 178, 139, 47], OperandSize::Qword)
}

fn pcmpgtw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 210], OperandSize::Dword)
}

fn pcmpgtw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 0], OperandSize::Dword)
}

fn pcmpgtw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 213], OperandSize::Qword)
}

fn pcmpgtw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 36, 155], OperandSize::Qword)
}

