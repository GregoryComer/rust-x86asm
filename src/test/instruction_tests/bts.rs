use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bts_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 219], OperandSize::Word)
}

fn bts_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(DI, 153, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 157, 153, 0], OperandSize::Word)
}

fn bts_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 205], OperandSize::Dword)
}

fn bts_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 59], OperandSize::Dword)
}

fn bts_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 245], OperandSize::Qword)
}

fn bts_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(RCX, 179074219, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 153, 171, 116, 172, 10], OperandSize::Qword)
}

fn bts_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 206], OperandSize::Word)
}

fn bts_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(SI, 13794, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 164, 226, 53], OperandSize::Word)
}

fn bts_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 222], OperandSize::Dword)
}

fn bts_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(EAX, 1990339351, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 176, 23, 43, 162, 118], OperandSize::Dword)
}

fn bts_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 228], OperandSize::Qword)
}

fn bts_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 1415441698, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 188, 185, 34, 237, 93, 84], OperandSize::Qword)
}

fn bts_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 171, 255], OperandSize::Qword)
}

fn bts_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 171, 12, 73], OperandSize::Qword)
}

fn bts_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(SI)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 238, 109], OperandSize::Word)
}

fn bts_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(SI, 5158, Some(OperandSize::Word), None)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 172, 38, 20, 41], OperandSize::Word)
}

fn bts_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BP)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 237, 74], OperandSize::Dword)
}

fn bts_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 81073257, Some(OperandSize::Word), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 44, 197, 105, 20, 213, 4, 105], OperandSize::Dword)
}

fn bts_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BX)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 235, 61], OperandSize::Qword)
}

fn bts_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 44, 135, 58], OperandSize::Qword)
}

fn bts_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ECX)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 233, 95], OperandSize::Word)
}

fn bts_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(BX, 9986, Some(OperandSize::Dword), None)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 175, 2, 39, 61], OperandSize::Word)
}

fn bts_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDI)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 239, 45], OperandSize::Dword)
}

fn bts_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 47, 4], OperandSize::Dword)
}

fn bts_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDI)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 239, 71], OperandSize::Qword)
}

fn bts_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 43, 100], OperandSize::Qword)
}

fn bts_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(RCX)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 233, 22], OperandSize::Qword)
}

fn bts_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 41, 82], OperandSize::Qword)
}

