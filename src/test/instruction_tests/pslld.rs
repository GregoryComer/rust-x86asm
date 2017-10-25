use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pslld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM6)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 246, 84], OperandSize::Dword)
}

fn pslld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM3)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 243, 109], OperandSize::Qword)
}

fn pslld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 243, 71], OperandSize::Dword)
}

fn pslld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 246, 81], OperandSize::Qword)
}

fn pslld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 208], OperandSize::Dword)
}

fn pslld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 52, 89], OperandSize::Dword)
}

fn pslld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 210], OperandSize::Qword)
}

fn pslld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 1512063095, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 132, 145, 119, 64, 32, 90], OperandSize::Qword)
}

fn pslld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 228], OperandSize::Dword)
}

fn pslld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 12, 214], OperandSize::Dword)
}

fn pslld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 215], OperandSize::Qword)
}

fn pslld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 12, 83], OperandSize::Qword)
}

