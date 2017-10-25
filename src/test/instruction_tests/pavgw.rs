use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pavgw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 250], OperandSize::Dword)
}

fn pavgw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 28, 190], OperandSize::Dword)
}

fn pavgw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 192], OperandSize::Qword)
}

fn pavgw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 28, 219], OperandSize::Qword)
}

fn pavgw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 219], OperandSize::Dword)
}

fn pavgw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 440689282, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 164, 120, 130, 98, 68, 26], OperandSize::Dword)
}

fn pavgw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 228], OperandSize::Qword)
}

fn pavgw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1270116885, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 4, 213, 21, 114, 180, 75], OperandSize::Qword)
}

